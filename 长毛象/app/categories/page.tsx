"use client";

import { useState } from "react";
import { useStore } from "@/lib/store";
import { generateId } from "@/lib/utils";
import Modal from "@/components/Modal";
import { IconPlus, IconEdit, IconTrash } from "@/components/icons";

const PRESET_COLORS = [
  "#3B82F6", "#8B5CF6", "#F59E0B", "#10B981", "#6366F1",
  "#EC4899", "#F97316", "#14B8A6", "#EF4444", "#6B7280",
];

const PRESET_ICONS = [
  "📱", "💻", "🎧", "📷", "🎮", "👔", "👗", "👟", "📚", "📖",
  "🍎", "🥤", "🍳", "🪑", "🛋️", "🔧", "✏️", "🎨", "⚽", "🎸",
  "💊", "🧴", "🪥", "🔑", "👜", "🧳", "📦", "💡", "🕶️", "⌚",
];

export default function CategoriesPage() {
  const { data, dispatch, ready } = useStore();
  const [modalOpen, setModalOpen] = useState(false);
  const [editId, setEditId] = useState<string | null>(null);
  const [name, setName] = useState("");
  const [icon, setIcon] = useState("📦");
  const [color, setColor] = useState(PRESET_COLORS[0]);
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
    setIcon("📦");
    setColor(PRESET_COLORS[0]);
    setModalOpen(true);
  }

  function openEdit(id: string) {
    const cat = data.categories.find((c) => c.id === id);
    if (!cat) return;
    setEditId(id);
    setName(cat.name);
    setIcon(cat.icon);
    setColor(cat.color);
    setModalOpen(true);
  }

  function handleSave() {
    if (!name.trim()) return;
    if (editId) {
      dispatch({
        type: "UPDATE_CATEGORY",
        payload: { id: editId, name: name.trim(), icon, color },
      });
    } else {
      dispatch({
        type: "ADD_CATEGORY",
        payload: { id: generateId(), name: name.trim(), icon, color },
      });
    }
    setModalOpen(false);
  }

  function handleDelete(id: string) {
    const usedCount = data.items.filter((i) => i.categoryId === id).length;
    if (usedCount > 0) {
      alert(`该分类下还有 ${usedCount} 件物品，请先移除或更改这些物品的分类。`);
      return;
    }
    dispatch({ type: "DELETE_CATEGORY", payload: id });
    setDeleteConfirm(null);
  }

  return (
    <div className="max-w-4xl mx-auto px-4 sm:px-6 py-6 sm:py-8">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">分类</h1>
          <p className="text-sm text-gray-500 mt-0.5">
            管理物品分类
          </p>
        </div>
        <button onClick={openNew} className="btn-primary">
          <IconPlus size={18} />
          <span className="hidden sm:inline">新增分类</span>
        </button>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
        {data.categories.map((cat) => {
          const count = data.items.filter(
            (i) => i.categoryId === cat.id
          ).length;
          return (
            <div
              key={cat.id}
              className="bg-white rounded-2xl border border-gray-100 p-4 flex items-center gap-4"
            >
              <div
                className="w-12 h-12 rounded-xl flex items-center justify-center text-2xl flex-shrink-0"
                style={{ backgroundColor: cat.color + "18" }}
              >
                {cat.icon}
              </div>
              <div className="flex-1 min-w-0">
                <h3 className="font-semibold text-gray-900 truncate">
                  {cat.name}
                </h3>
                <p className="text-xs text-gray-400">{count} 件物品</p>
              </div>
              <div className="flex gap-1">
                <button
                  onClick={() => openEdit(cat.id)}
                  className="p-2 rounded-lg hover:bg-gray-100 text-gray-400 hover:text-gray-600 transition-colors"
                >
                  <IconEdit size={16} />
                </button>
                <button
                  onClick={() => setDeleteConfirm(cat.id)}
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
        title={editId ? "编辑分类" : "新增分类"}
      >
        <div className="space-y-5">
          <div>
            <label className="label">名称</label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="分类名称"
              className="input"
              autoFocus
            />
          </div>

          <div>
            <label className="label">图标</label>
            <div className="flex flex-wrap gap-2">
              {PRESET_ICONS.map((ic) => (
                <button
                  key={ic}
                  type="button"
                  onClick={() => setIcon(ic)}
                  className={`w-10 h-10 rounded-xl text-xl flex items-center justify-center transition-all ${
                    icon === ic
                      ? "bg-primary-100 ring-2 ring-primary-500 scale-110"
                      : "bg-gray-50 hover:bg-gray-100"
                  }`}
                >
                  {ic}
                </button>
              ))}
            </div>
          </div>

          <div>
            <label className="label">颜色</label>
            <div className="flex flex-wrap gap-2">
              {PRESET_COLORS.map((c) => (
                <button
                  key={c}
                  type="button"
                  onClick={() => setColor(c)}
                  className={`w-10 h-10 rounded-xl transition-all ${
                    color === c ? "ring-2 ring-offset-2 ring-gray-400 scale-110" : ""
                  }`}
                  style={{ backgroundColor: c }}
                />
              ))}
            </div>
          </div>

          {/* Preview */}
          <div>
            <label className="label">预览</label>
            <div className="flex items-center gap-3 bg-gray-50 rounded-xl p-3">
              <div
                className="w-10 h-10 rounded-xl flex items-center justify-center text-xl"
                style={{ backgroundColor: color + "18" }}
              >
                {icon}
              </div>
              <span className="font-medium text-gray-900">
                {name || "分类名称"}
              </span>
            </div>
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
            <p className="text-sm text-gray-500 mb-6">确定要删除该分类吗？</p>
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
