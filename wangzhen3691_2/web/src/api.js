const B = '' // base URL — empty in dev (Vite proxy), set in prod if needed

const json = (r) => r.json()

export const fetchStats   = () => fetch(`${B}/api/stats`).then(json)
export const fetchAlerts  = () => fetch(`${B}/api/alerts`).then(json)
export const fetchRules   = () => fetch(`${B}/api/rules`).then(json)
export const fetchPlugins = () => fetch(`${B}/api/plugins`).then(json)

export const scanContent = (text, source = 'manual') =>
  fetch(`${B}/api/scan`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ text, source }),
  }).then(json)

export const createRule = (rule) =>
  fetch(`${B}/api/rules`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(rule),
  }).then(json)

export const updateRule = (id, rule) =>
  fetch(`${B}/api/rules/${id}`, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(rule),
  }).then(json)

export const deleteRule = (id) =>
  fetch(`${B}/api/rules/${id}`, { method: 'DELETE' }).then(json)

export const togglePlugin = (name) =>
  fetch(`${B}/api/plugins/${encodeURIComponent(name)}/toggle`, {
    method: 'POST',
  }).then(json)

export const testRule = (pattern, text) =>
  fetch(`${B}/api/rules/test`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ pattern, text }),
  }).then(json)
