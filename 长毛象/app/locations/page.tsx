"use client";

import { useState } from "react";
import { useStore } from "@/lib/store";
import { generateId } from "@/lib/utils";
import Modal from "@/components/Modal";
import { IconPlus, IconEdit, IconTrash, IconMapPin } from "@/components/icons";

export default function LocationsPage() {
  const { data, dispatch, ready } = useStore();
  const [modalOpen, setModalOpen] = useState(false);
  const [editId, setEditId] = useState<string | null>(null);
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [deleteConfirm, setDeleteConfirm] = useState<string | null>(null);

  if (!ready) {
    return (
      <div className="flex items-center justify-center h-screen">
        <div className="w-8 h-8 border-2 border-primary-600 border-t-transparent rounded-full animate-spin" />
      </div>
    );
  }

  function openNew() {
    setEditId(null);
    setName("");
    setDescription("");
    setModalOpen(true);
  }

  function openEdit(id: string) {
    const loc = data.locations.find((l) => l.id === id);
    if (!loc) return;
    setEditId(id);
    setName(loc.name);
    setDescription(loc.description || "");
    setModalOpen(true);
  }

  function handleSave() {
    if (!name.trim()) return;
    if (editId) {
      dispatch({
        type: "UPDATE_LOCATION",
        payload: {
          id: editId,
          name: name.trim(),
          description: description.trim() || undefined,
        },
      });
    } else {
      dispatch({
        type: "ADD_LOCATION",
        payload: {
          id: generateId(),
          name: name.trim(),
          description: description.trim() || undefined,
        },
      });
    }
    setModalOpen(false);
  }

  function handleDelete(id: string) {
    const usedCount = data.items.filter((i) => i.locationId === id).length;
    if (usedCount > 0) {
      alert(
        `该位置下还有 ${usedCount} 件物品，请先移除或更改这些物品的位置。`
      );
      return;
    }
    dispatch({ type: "DELETE_LOCATION", payload: id });
    setDeleteConfirm(null);
  }

  return (
    <div className="max-w-4xl mx-auto px-4 sm:px-6 py-6 sm:py-8">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">位置</h1>
          <p className="text-sm text-gray-500 mt-0.5">管理物品存放位置</p>
        </div>
        <button onClick={openNew} className="btn-primary">
          <IconPlus size={18} />
          <span className="hidden sm:inline">新增位置</span>
        </button>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
        {data.locations.map((loc) => {
          const count = data.items.filter(
            (i) => i.locationId === loc.id
          ).length;
          return (
            <div
              key={loc.id}
              className="bg-white rounded-2xl border border-gray-100 p-4 flex items-center gap-4"
            >
              <div className="w-12 h-12 rounded-xl bg-pink-50 text-pink-500 flex items-center justify-center flex-shrink-0">
                <IconMapPin size={22} />
              </div>
              <div className="flex-1 min-w-0">
                <h3 className="font-semibold text-gray-900 truncate">
                  {loc.name}
                </h3>
                {loc.description && (
                  <p className="text-xs text-gray-400 truncate">
                    {loc.description}
                  </p>
                )}
                <p className="text-xs text-gray-400 mt-0.5">
                  {count} 件物品
                </p>
              </div>
              <div className="flex gap-1">
                <button
                  onClick={() => openEdit(loc.id)}
                  className="p-2 rounded-lg hover:bg-gray-100 text-gray-400 hover:text-gray-600 transition-colors"
                >
                  <IconEdit size={16} />
                </button>
                <button
                  onClick={() => setDeleteConfirm(loc.id)}
                  className="p-2 rounded-lg hover:bg-red-50 text-gray-400 hover:text-red-500 transition-colors"
                >
                  <IconTrash size={16} />
                </button>
              </div>
            </div>
          );
        })}
      </div>

      {/* Add/Edit Modal */}
      <Modal
        open={modalOpen}
        onClose={() => setModalOpen(false)}
        title={editId ? "编辑位置" : "新增位置"}
      >
        <div className="space-y-5">
          <div>
            <label className="label">名称</label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="例如：卧室、客厅"
              className="input"
              autoFocus
            />
          </div>
          <div>
            <label className="label">描述（可选）</label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="添加一些描述..."
              rows={2}
              className="input resize-none"
            />
          </div>
          <div className="flex gap-3 pt-2">
            <button
              onClick={() => setModalOpen(false)}
              className="btn-secondary flex-1"
            >
              取消
            </button>
            <button
              onClick={handleSave}
              disabled={!name.trim()}
              className="btn-primary flex-1"
            >
              保存
            </button>
          </div>
        </div>
      </Modal>

      {/* Delete Confirmation */}
      {deleteConfirm && (
        <div className="fixed inset-0 z-50 flex items-center justify-center">
          <div
            className="fixed inset-0 bg-black/40 backdrop-blur-sm"
            onClick={() => setDeleteConfirm(null)}
          />
          <div className="relative bg-white rounded-2xl p-6 mx-4 max-w-sm w-full shadow-xl">
            <h3 className="text-lg font-semibold text-gray-900 mb-2">
              确认删除
            </h3>
            <p className="text-sm text-gray-500 mb-6">确定要删除该位置吗？</p>
            <div className="flex gap-3">
              <button
                onClick={() => setDeleteConfirm(null)}
                className="btn-secondary flex-1"
              >
                取消
              </button>
              <button
                onClick={() => handleDelete(deleteConfirm)}
                className="btn-danger flex-1"
              >
                删除
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
