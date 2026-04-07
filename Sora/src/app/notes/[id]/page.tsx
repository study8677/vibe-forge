import { prisma } from '@/lib/prisma'
import { notFound } from 'next/navigation'
import NoteEditor from '@/components/NoteEditor'

interface Props {
  params: { id: string }
}

export default async function NotePage({ params }: Props) {
  const note = await prisma.note.findUnique({
    where: { id: params.id },
  })

  if (!note) {
    notFound()
  }

  return (
    <NoteEditor
      initialNote={{
        id: note.id,
        title: note.title,
        content: note.content,
        status: note.status,
        emoji: note.emoji,
        pinned: note.pinned,
        createdAt: note.createdAt.toISOString(),
        updatedAt: note.updatedAt.toISOString(),
      }}
    />
  )
}
