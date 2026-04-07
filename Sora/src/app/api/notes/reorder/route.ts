import { NextRequest, NextResponse } from 'next/server'
import { prisma } from '@/lib/prisma'

export async function PATCH(req: NextRequest) {
  const { noteId, newStatus, newIndex } = await req.json()

  await prisma.note.update({
    where: { id: noteId },
    data: { status: newStatus },
  })

  const notes = await prisma.note.findMany({
    where: { status: newStatus },
    orderBy: { order: 'asc' },
  })

  const ordered = notes.filter((n) => n.id !== noteId)
  const moved = notes.find((n) => n.id === noteId)!
  ordered.splice(newIndex, 0, moved)

  await prisma.$transaction(
    ordered.map((note, index) =>
      prisma.note.update({
        where: { id: note.id },
        data: { order: index },
      })
    )
  )

  return NextResponse.json({ success: true })
}
