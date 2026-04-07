import { NextRequest, NextResponse } from 'next/server'
import { prisma } from '@/lib/prisma'

interface Params {
  params: { id: string }
}

export async function GET(_req: NextRequest, { params }: Params) {
  const note = await prisma.note.findUnique({ where: { id: params.id } })
  if (!note) {
    return NextResponse.json({ error: 'Note not found' }, { status: 404 })
  }
  return NextResponse.json(note)
}

export async function PATCH(req: NextRequest, { params }: Params) {
  const body = await req.json()

  const data: Record<string, unknown> = {}
  if (body.title !== undefined) data.title = body.title
  if (body.content !== undefined) data.content = body.content
  if (body.status !== undefined) data.status = body.status
  if (body.emoji !== undefined) data.emoji = body.emoji
  if (body.pinned !== undefined) data.pinned = body.pinned
  if (body.order !== undefined) data.order = body.order

  try {
    const note = await prisma.note.update({
      where: { id: params.id },
      data,
    })
    return NextResponse.json(note)
  } catch {
    return NextResponse.json({ error: 'Note not found' }, { status: 404 })
  }
}

export async function DELETE(_req: NextRequest, { params }: Params) {
  try {
    await prisma.note.delete({ where: { id: params.id } })
    return NextResponse.json({ success: true })
  } catch {
    return NextResponse.json({ error: 'Note not found' }, { status: 404 })
  }
}
