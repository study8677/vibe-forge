"use client";

import { useState, useRef } from "react";
import { useRouter } from "next/navigation";
import Link from "next/link";
import { useStore } from "@/lib/store";
import { generateId, compressImage } from "@/lib/utils";
import { IconChevronLeft, IconCamera, IconX } from "@/components/icons";

export default function NewItemPage() {
  const router = useRouter();
  const { data, dispatch } = useStore();
  const fileInputRef = useRef<HTMLInputElement>(null);

  const [name, setName] = useState("");
  const [categoryId, setCategoryId] = useState(data.categories[0]?.id || "");
  const [locationId, setLocationId] = useState(data.locations[0]?.id || "");
  const [quantity, setQuantity] = useState("1");
  const [price, setPrice] = useState("");
  const [purchaseDate, setPurchaseDate] = useState("");
  const [notes, setNotes] = useState("");
  const [imageData, setImageData] = useState("");
  const [saving, setSaving] = useState(false);

  async function handleImageChange(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    if (!file) return;
    try {
      const compressed = await compressImage(file);
      setImageData(compressed);
    } catch {
      alert("图片处理失败，请重试");
    }
  }

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (!name.trim()) return;

    setSaving(true);
    const now = new Date().toISOString();
    dispatch({
      type: "ADD_ITEM",
      payload: {
        id: generateId(),
        name: name.trim(),
        categoryId,
        locationId,
        quantity: Math.max(1, parseInt(quantity) || 1),
        price: price ? parseFloat(price) : undefined,
        purchaseDate: purchaseDate || undefined,
        notes: notes.trim() || undefined,
        imageData: imageData || undefined,
        createdAt: now,
        updatedAt: now,
      },
    });
    router.push("/items");
  }

  return (
    <div className="max-w-2xl mx-auto px-4 sm:px-6 py-6 sm:py-8">
      {/* Header */}
      <div className="flex items-center gap-3 mb-6">
        <Link
          href="/items"
          className="p-2 -ml-2 rounded-xl hover:bg-gray-100 transition-colors text-gray-600"
        >
          <IconChevronLeft size={20} />
        </Link>
        <h1 className="text-xl font-bold text-gray-900">添加物品</h1>
      </div>

      <form onSubmit={handleSubmit} className="space-y-5">
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

        {/* Name */}
        <div>
          <label className="label" htmlFor="name">
            名称 <span className="text-red-500">*</span>
          </label>
          <input
            id="name"
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="物品名称"
            className="input"
            required
            autoFocus
          />
        </div>

        {/* Category & Location */}
        <div className="grid grid-cols-2 gap-3">
          <div>
            <label className="label" htmlFor="category">
              分类
            </label>
            <select
              id="category"
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
            <label className="label" htmlFor="location">
              存放位置
            </label>
            <select
              id="location"
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

        {/* Quantity & Price */}
        <div className="grid grid-cols-2 gap-3">
          <div>
            <label className="label" htmlFor="quantity">
              数量
            </label>
            <input
              id="quantity"
              type="number"
              value={quantity}
              onChange={(e) => setQuantity(e.target.value)}
              min="1"
              className="input"
            />
          </div>
          <div>
            <label className="label" htmlFor="price">
              价格 (元)
            </label>
            <input
              id="price"
              type="number"
              value={price}
              onChange={(e) => setPrice(e.target.value)}
              placeholder="0.00"
              min="0"
              step="0.01"
              className="input"
            />
          </div>
        </div>

        {/* Purchase Date */}
        <div>
          <label className="label" htmlFor="purchaseDate">
            购买日期
          </label>
          <input
            id="purchaseDate"
            type="date"
            value={purchaseDate}
            onChange={(e) => setPurchaseDate(e.target.value)}
            className="input"
          />
        </div>

        {/* Notes */}
        <div>
          <label className="label" htmlFor="notes">
            备注
          </label>
          <textarea
            id="notes"
            value={notes}
            onChange={(e) => setNotes(e.target.value)}
            placeholder="添加一些备注信息..."
            rows={3}
            className="input resize-none"
          />
        </div>

        {/* Submit */}
        <div className="flex gap-3 pt-2">
          <Link href="/items" className="btn-secondary flex-1">
            取消
          </Link>
          <button
            type="submit"
            disabled={!name.trim() || saving}
            className="btn-primary flex-1"
          >
            {saving ? "保存中..." : "保存"}
          </button>
        </div>
      </form>
    </div>
  );
}
