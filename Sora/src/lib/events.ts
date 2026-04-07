const bus = typeof window !== 'undefined' ? new EventTarget() : null

export function emitNoteChange() {
  bus?.dispatchEvent(new Event('note-change'))
}

export function onNoteChange(callback: () => void): () => void {
  if (!bus) return () => {}
  bus.addEventListener('note-change', callback)
  return () => bus.removeEventListener('note-change', callback)
}
