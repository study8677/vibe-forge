"use client";

import { useState, useRef, use } from "react";
import { useRouter } from "next/navigation";
import Link from "next/link";
import { useStore } from "@/lib/store";
import { formatPrice, formatDate, compressImage } from "@/lib/utils";
import {
  IconChevronLeft,
  IconEdit,
  IconTrash,
  IconCamera,
  IconX,
} from "@/components/icons";

export default function ItemDetailPage({
  params,
}: {
  params: Promise<{ id: string }>;
}) {
  const { id } = use(params);
  const router = useRouter();
  const { data, dispatch } = useStore();
  const fileInputRef = useRef<HTMLInputElement>(null);

  const item = data.items.find((i) => i.id === id);
  const [editing, setEditing] = useState(false);

  // Edit form state
  const [name, setName] = useState(item?.name || "");
  const [categoryId, setCategoryId] = useState(item?.categoryId || "");
  const [locationId, setLocationId] = useState(item?.locationId || "");
  const [quantity, setQuantity] = useState(String(item?.quantity || 1));
  const [price, setPrice] = useState(
    item?.price != null ? String(item.price) : ""
  );
  const [purchaseDate, setPurchaseDate] = useState(item?.purchaseDate || "");
  const [notes, setNotes] = useState(item?.notes || "");
  const [imageData, setImageData] = useState(item?.imageData || "");
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);

  if (!item) {
    return (
      <div className="max-w-2xl mx-auto px-4 sm:px-6 py-6 sm:py-8">
        <div className="flex items-center gap-3 mb-6">
          <Link
            href="/items"
            className="p-2 -ml-2 rounded-xl hover:bg-gray-100 transition-colors text-gray-600"
          >
            <IconChevronLeft size={20} />
          </Link>
          <h1 className="text-xl font-bold text-gray-900">物品未找到</h1>
        </div>
        <p className="text-gray-500">该物品不存在或已被删除。</p>
      </div>
    );
  }

  const category = data.categories.find((c) => c.id === item.categoryId);
  const location = data.locations.find((l) => l.id === item.locationId);

  function handleDelete() {
    dispatch({ type: "DELETE_ITEM", payload: item!.id });
    router.push("/items");
  }

  function handleSave(e: React.FormEvent) {
    e.preventDefault();
    if (!name.trim()) return;

    dispatch({
      type: "UPDATE_ITEM",
      payload: {
        ...item!,
        name: name.trim(),
        categoryId,
        locationId,
        quantity: Math.max(1, parseInt(quantity) || 1),
        price: price ? parseFloat(price) : undefined,
        purchaseDate: purchaseDate || undefined,
        notes: notes.trim() || undefined,
        imageData: imageData || undefined,
        updatedAt: new Date().toISOString(),
      },
    });
    setEditing(false);
  }

  async function handleImageChange(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    if (!file) return;
    try {
      const compressed = await compressImage(file);
      setImageData(compressed);
    } catch {
      alert("图片处理失败");
    }
  }

  if (editing) {
    return (
      <div className="max-w-2xl mx-auto px-4 sm:px-6 py-6 sm:py-8">
        <div className="flex items-center gap-3 mb-6">
          <button
            onClick={() => setEditing(false)}
            className="p-2 -ml-2 rounded-xl hover:bg-gray-100 transition-colors text-gray-600"
          >
            <IconChevronLeft size={20} />
          </button>
          <h1 className="text-xl font-bold text-gray-900">编辑物品</h1>
        </div>

        <form onSubmit={handleSave} className="space-y-5">
          {/* Image */}
          <div>
            <label className="label">照片</label>
            {imageData ? (
              <div className="relative w-full aspect-video rounded-xl overflow-hidden bg-gray-100">
                <img
                  src={imageData}
                  alt="预览"
                  className="w-full h-full object-cover"
                />
                <button
                  type="button"
                  onClick={() => setImageData("")}
                  className="absolute top-2 right-2 w-8 h-8 bg-black/50 text-white rounded-full flex items-center justify-center hover:bg-black/70"
                >
                  <IconX size={16} />
                </button>
              </div>
            ) : (
              <button
                type="button"
                onClick={() => fileInputRef.current?.click()}
                className="w-full aspect-video rounded-xl border-2 border-dashed border-gray-200 hover:border-primary-300 flex flex-col items-center justify-center gap-2 text-gray-400 hover:text-primary-500 transition-colors bg-gray-50"
              >
                <IconCamera size={32} />
                <span className="text-sm">点击拍照或选择图片</span>
              </button>
            )}
            <input
              ref={fileInputRef}
              type="file"
              accept="image/*"
              capture="environment"
              onChange={handleImageChange}
              className="hidden"
            />
          </div>

          <div>
            <label className="label" htmlFor="name">名称</label>
            <input
              id="name"
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="input"
              required
            />
          </div>

          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="label">分类</label>
              <select
                value={categoryId}
                onChange={(e) => setCategoryId(e.target.value)}
                className="input"
              >
                {data.categories.map((c) => (
                  <option key={c.id} value={c.id}>
                    {c.icon} {c.name}
                  </option>
                ))}
              </select>
            </div>
            <div>
              <label className="label">位置</label>
              <select
                value={locationId}
                onChange={(e) => setLocationId(e.target.value)}
                className="input"
              >
                {data.locations.map((l) => (
                  <option key={l.id} value={l.id}>
                    {l.name}
                  </option>
                ))}
              </select>
            </div>
          </div>

          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="label">数量</label>
              <input
                type="number"
                value={quantity}
                onChange={(e) => setQuantity(e.target.value)}
                min="1"
                className="input"
              />
            </div>
            <div>
              <label className="label">价格 (元)</label>
              <input
                type="number"
                value={price}
                onChange={(e) => setPrice(e.target.value)}
                min="0"
                step="0.01"
                className="input"
              />
            </div>
          </div>

          <div>
            <label className="label">购买日期</label>
            <input
              type="date"
              value={purchaseDate}
              onChange={(e) => setPurchaseDate(e.target.value)}
              className="input"
            />
          </div>

          <div>
            <label className="label">备注</label>
            <textarea
              value={notes}
              onChange={(e) => setNotes(e.target.value)}
              rows={3}
              className="input resize-none"
            />
          </div>

          <div className="flex gap-3 pt-2">
            <button
              type="button"
              onClick={() => setEditing(false)}
              className="btn-secondary flex-1"
            >
              取消
            </button>
            <button
              type="submit"
              disabled={!name.trim()}
              className="btn-primary flex-1"
            >
              保存
            </button>
          </div>
        </form>
      </div>
    );
  }

  return (
    <div className="max-w-2xl mx-auto px-4 sm:px-6 py-6 sm:py-8">
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center gap-3">
          <Link
            href="/items"
            className="p-2 -ml-2 rounded-xl hover:bg-gray-100 transition-colors text-gray-600"
          >
            <IconChevronLeft size={20} />
          </Link>
          <h1 className="text-xl font-bold text-gray-900 truncate">
            {item.name}
          </h1>
        </div>
        <div className="flex gap-2">
          <button
            onClick={() => setEditing(true)}
            className="btn-secondary"
          >
            <IconEdit size={16} />
            <span className="hidden sm:inline">编辑</span>
          </button>
          <button
            onClick={() => setShowDeleteConfirm(true)}
            className="p-2.5 rounded-xl border border-gray-200 text-red-500 hover:bg-red-50 hover:border-red-200 transition-colors"
          >
            <IconTrash size={16} />
          </button>
        </div>
      </div>

      {/* Image */}
      {item.imageData && (
        <div className="rounded-2xl overflow-hidden mb-6 bg-gray-100">
          <img
            src={item.imageData}
            alt={item.name}
            className="w-full max-h-80 object-cover"
          />
        </div>
      )}

      {/* Details */}
      <div className="bg-white rounded-2xl border border-gray-100 overflow-hidden">
        <div className="divide-y divide-gray-50">
          <DetailRow label="分类">
            {category ? (
              <span
                className="inline-flex items-center gap-1 text-sm px-2.5 py-0.5 rounded-full font-medium"
                style={{
                  backgroundColor: category.color + "18",
                  color: category.color,
                }}
              >
                {category.icon} {category.name}
              </span>
            ) : (
              <span className="text-gray-400">未分类</span>
            )}
          </DetailRow>
          <DetailRow label="存放位置">
            {location?.name || "未设置"}
          </DetailRow>
          <DetailRow label="数量">{item.quantity}</DetailRow>
          {item.price != null && item.price > 0 && (
            <DetailRow label="价格">
              {formatPrice(item.price)}
              {item.quantity > 1 && (
                <span className="text-gray-400 text-xs ml-2">
                  (总计 {formatPrice(item.price * item.quantity)})
                </span>
              )}
            </DetailRow>
          )}
          {item.purchaseDate && (
            <DetailRow label="购买日期">{item.purchaseDate}</DetailRow>
          )}
          {item.notes && (
            <DetailRow label="备注">
              <span className="whitespace-pre-wrap">{item.notes}</span>
            </DetailRow>
          )}
          <DetailRow label="添加时间">{formatDate(item.createdAt)}</DetailRow>
          <DetailRow label="更新时间">{formatDate(item.updatedAt)}</DetailRow>
        </div>
      </div>

      {/* Delete Confirmation */}
      {showDeleteConfirm && (
        <div className="fixed inset-0 z-50 flex items-center justify-center">
          <div
            className="fixed inset-0 bg-black/40 backdrop-blur-sm"
            onClick={() => setShowDeleteConfirm(false)}
          />
          <div className="relative bg-white rounded-2xl p-6 mx-4 max-w-sm w-full shadow-xl">
            <h3 className="text-lg font-semibold text-gray-900 mb-2">
              确认删除
            </h3>
            <p className="text-sm text-gray-500 mb-6">
              确定要删除「{item.name}」吗？此操作无法撤销。
            </p>
            <div className="flex gap-3">
              <button
                onClick={() => setShowDeleteConfirm(false)}
                className="btn-secondary flex-1"
              >
                取消
              </button>
              <button onClick={handleDelete} className="btn-danger flex-1">
                删除
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

function DetailRow({
  label,
  children,
}: {
  label: string;
  children: React.ReactNode;
}) {
  return (
    <div className="flex items-start justify-between px-5 py-3.5">
      <span className="text-sm text-gray-500 flex-shrink-0 mr-4">{label}</span>
      <span className="text-sm text-gray-900 text-right">{children}</span>
    </div>
  );
}
