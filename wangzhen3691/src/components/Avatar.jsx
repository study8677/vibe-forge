export default function Avatar({ user, size = 25 }) {
  return (
    <div
      className="rounded-full shrink-0 flex items-center justify-center font-bold text-white select-none"
      style={{ width: size, height: size, background: user.color, fontSize: size * 0.44 }}
      title={user.name}
    >
      {user.letter}
    </div>
  )
}
