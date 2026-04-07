import { NextRequest, NextResponse } from 'next/server'
import { prisma } from '@/lib/prisma'

export async function GET(req: NextRequest) {
  const { searchParams } = req.nextUrl
  const status = searchParams.get('status')
  const search = searchParams.get('search')

  const where: Record<string, unknown> = {}

  if (status) {
    where.status = status
  }

  if (search) {
    where.OR = [
      { title: { contains: search } },
      { content: { contains: search } },
    ]
  }

  const notes = await prisma.note.findMany({
    where,
    orderBy: [{ pinned: 'desc' }, { updatedAt: 'desc' }],
  })

  return NextResponse.json(notes)
}

export async function POST(req: NextRequest) {
  const body = await req.json()

  const maxOrder = await prisma.note.aggregate({
    _max: { order: true },
    where: { status: body.status ?? 'backlog' },
  })

  const note = await prisma.note.create({
    data: {
      title: body.title ?? '',
      content: body.content ?? '',
      status: body.status ?? 'backlog',
      emoji: body.emoji ?? null,
      order: (maxOrder._max.order ?? -1) + 1,
    },
  })

  return NextResponse.json(note, { status: 201 })
}
