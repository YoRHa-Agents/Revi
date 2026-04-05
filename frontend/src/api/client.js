import { reactive } from 'vue'

const serverState = reactive({
  base: localStorage.getItem('revi_api_base') || import.meta.env.VITE_API_BASE || 'http://localhost:8000',
})

export function getApiBase() {
  return serverState.base
}

export function setApiBase(url) {
  const normalized = url.replace(/\/+$/, '')
  serverState.base = normalized
  localStorage.setItem('revi_api_base', normalized)
}

async function request(method, path, body) {
  const opts = {
    method,
    headers: body ? { 'Content-Type': 'application/json' } : {},
    body: body ? JSON.stringify(body) : undefined,
  }
  const res = await fetch(serverState.base + path, opts)
  if (!res.ok) {
    const text = await res.text().catch(() => '')
    throw new Error(`${method} ${path} → ${res.status}: ${text}`)
  }
  const ct = res.headers.get('content-type') || ''
  if (ct.includes('application/json')) return res.json()
  return null
}

export const api = {
  getReviews:      ()        => request('GET',   '/api/reviews'),
  getReview:       (id)      => request('GET',   `/api/reviews/${id}`),
  getComments:     (id)      => request('GET',   `/api/comments/${id}`),
  addComment:      (id, b)   => request('POST',  `/api/comments/${id}`, b),
  resolveComment:  (id, cid) => request('PATCH', `/api/comments/${id}/${cid}/resolve`),
  archiveResolved: (id)      => request('POST',  `/api/archive/${id}`),
  getArchive:      (id)      => request('GET',   `/api/archive/${id}`),
  exportForAgent:  (id)      => request('GET',   `/api/export/${id}`),
  upload: async (fd) => {
    const res = await fetch(serverState.base + '/api/upload', { method: 'POST', body: fd })
    if (!res.ok) {
      const text = await res.text().catch(() => '')
      throw new Error(`POST /api/upload → ${res.status}: ${text}`)
    }
    return res.json()
  },
  updateType: (id, type) => request('PATCH', `/api/reviews/${id}`, { type }),
  getConfig:       ()        => request('GET',   '/api/config'),
  updateConfig:    (body)    => request('PATCH', '/api/config', body),
}
