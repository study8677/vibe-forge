import struct
from typing import Dict, List, Optional

from .base import ExtractedEntry, FormatHandler


class BinaryHandler(FormatHandler):
    """Handles binary files with pointer tables and encoded strings."""

    def __init__(self, config: dict):
        super().__init__(config)
        bc = config.get("binary", {})
        self.endian: str = bc.get("endian", "little")
        self.string_encoding: str = bc.get("string_encoding", "utf-8")
        self.null_terminated: bool = bc.get("null_terminated", True)

        pt = bc.get("pointer_table", {})
        self.pt_offset: int = self._parse_int(pt.get("offset", 0))
        self.pt_count: Optional[int] = self._parse_int(pt.get("count")) if pt.get("count") is not None else None
        self.pt_count_offset: Optional[int] = (
            self._parse_int(pt.get("count_offset"))
            if pt.get("count_offset") is not None
            else None
        )
        self.pt_entry_size: int = self._parse_int(pt.get("entry_size", 4))

        # Direct string table (no pointers)
        st = bc.get("string_table", {})
        self.st_offset: int = self._parse_int(st.get("offset", 0))
        self.st_count: Optional[int] = self._parse_int(st.get("count")) if st.get("count") is not None else None
        self.st_fixed_length: Optional[int] = (
            self._parse_int(st.get("fixed_length"))
            if st.get("fixed_length") is not None
            else None
        )

        self.use_pointer_table: bool = "pointer_table" in bc

    @staticmethod
    def _parse_int(value) -> Optional[int]:
        if value is None:
            return None
        if isinstance(value, int):
            return value
        if isinstance(value, str):
            return int(value, 0)  # handles 0x prefix
        return int(value)

    @property
    def _byte_order(self) -> str:
        return "<" if self.endian == "little" else ">"

    @property
    def _ptr_format(self) -> str:
        fmt_map = {1: "B", 2: "H", 4: "I", 8: "Q"}
        return self._byte_order + fmt_map.get(self.pt_entry_size, "I")

    def extract(self, file_path: str) -> List[ExtractedEntry]:
        with open(file_path, "rb") as f:
            data = f.read()

        if self.use_pointer_table:
            return self._extract_pointer_table(data)
        else:
            return self._extract_string_table(data)

    def inject(self, file_path: str, entries: Dict[str, str], output_path: Optional[str] = None):
        with open(file_path, "rb") as f:
            data = bytearray(f.read())

        if self.use_pointer_table:
            data = self._inject_pointer_table(data, entries)
        else:
            data = self._inject_string_table(data, entries)

        dest = output_path or file_path
        with open(dest, "wb") as f:
            f.write(data)

    # ── Pointer table ──

    def _get_count(self, data: bytes) -> int:
        if self.pt_count is not None:
            return self.pt_count
        if self.pt_count_offset is not None:
            fmt = self._byte_order + "I"
            return struct.unpack_from(fmt, data, self.pt_count_offset)[0]
        raise ValueError("Binary format needs either count or count_offset")

    def _extract_pointer_table(self, data: bytes) -> List[ExtractedEntry]:
        count = self._get_count(data)
        results = []

        for i in range(count):
            ptr_pos = self.pt_offset + i * self.pt_entry_size
            if ptr_pos + self.pt_entry_size > len(data):
                break
            ptr = struct.unpack_from(self._ptr_format, data, ptr_pos)[0]
            if ptr >= len(data):
                continue

            text = self._read_string(data, ptr)
            results.append(ExtractedEntry(
                key=f"str_{i:04d}",
                value=text,
                line_number=ptr,  # store offset as line_number for reference
            ))

        return results

    def _inject_pointer_table(self, data: bytearray, entries: Dict[str, str]) -> bytearray:
        count = self._get_count(data)
        # Collect all pointer info
        ptrs = []
        for i in range(count):
            ptr_pos = self.pt_offset + i * self.pt_entry_size
            if ptr_pos + self.pt_entry_size > len(data):
                break
            ptr = struct.unpack_from(self._ptr_format, data, ptr_pos)[0]
            old_text = self._read_string(data, ptr)
            key = f"str_{i:04d}"
            new_text = entries.get(key, old_text)
            ptrs.append((i, ptr_pos, ptr, old_text, new_text))

        # Rebuild string section: find the start of string data
        if not ptrs:
            return data
        string_start = min(p[2] for p in ptrs)
        header = data[:string_start]
        new_strings = bytearray()
        new_ptrs = []

        for i, ptr_pos, old_ptr, old_text, new_text in ptrs:
            new_ptr = string_start + len(new_strings)
            encoded = new_text.encode(self.string_encoding)
            if self.null_terminated:
                encoded += b"\x00"
            new_strings.extend(encoded)
            new_ptrs.append((ptr_pos, new_ptr))

        result = bytearray(header) + new_strings
        for ptr_pos, new_ptr in new_ptrs:
            struct.pack_into(self._ptr_format, result, ptr_pos, new_ptr)

        return result

    # ── String table (fixed or null-terminated) ──

    def _extract_string_table(self, data: bytes) -> List[ExtractedEntry]:
        results = []
        pos = self.st_offset
        count = self.st_count or 9999

        for i in range(count):
            if pos >= len(data):
                break
            if self.st_fixed_length:
                end = min(pos + self.st_fixed_length, len(data))
                raw = data[pos:end]
                if self.null_terminated:
                    null_idx = raw.find(b"\x00")
                    if null_idx >= 0:
                        raw = raw[:null_idx]
                text = raw.decode(self.string_encoding, errors="replace")
                results.append(ExtractedEntry(key=f"str_{i:04d}", value=text, line_number=pos))
                pos += self.st_fixed_length
            else:
                text = self._read_string(data, pos)
                if not text and self.null_terminated:
                    break
                encoded_len = len(text.encode(self.string_encoding))
                results.append(ExtractedEntry(key=f"str_{i:04d}", value=text, line_number=pos))
                pos += encoded_len + (1 if self.null_terminated else 0)

        return results

    def _inject_string_table(self, data: bytearray, entries: Dict[str, str]) -> bytearray:
        if self.st_fixed_length:
            # Fixed-length: overwrite in place
            pos = self.st_offset
            count = self.st_count or 9999
            for i in range(count):
                if pos + self.st_fixed_length > len(data):
                    break
                key = f"str_{i:04d}"
                if key in entries:
                    encoded = entries[key].encode(self.string_encoding)
                    padded = encoded[:self.st_fixed_length].ljust(self.st_fixed_length, b"\x00")
                    data[pos:pos + self.st_fixed_length] = padded
                pos += self.st_fixed_length
            return data
        else:
            # Variable length: rebuild
            header = bytearray(data[:self.st_offset])
            extracted = self._extract_string_table(bytes(data))
            new_section = bytearray()
            for entry in extracted:
                text = entries.get(entry.key, entry.value)
                encoded = text.encode(self.string_encoding)
                new_section.extend(encoded)
                if self.null_terminated:
                    new_section.append(0)
            return header + new_section

    # ── Helpers ──

    def _read_string(self, data: bytes, offset: int) -> str:
        if offset >= len(data):
            return ""
        if self.null_terminated:
            end = data.index(b"\x00", offset) if b"\x00" in data[offset:] else len(data)
            return data[offset:end].decode(self.string_encoding, errors="replace")
        return ""
