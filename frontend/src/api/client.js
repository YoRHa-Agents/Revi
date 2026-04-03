// The Rust `revi` server is the only supported API runtime.
const BASE = import.meta.env.VITE_API_BASE ?? 'http://localhost:8000'

async function request(method, path, body) {
  const opts = {
    method,
    headers: body ? { 'Content-Type': 'application/json' } : {},
    body: body ? JSON.stringify(body) : undefined,
  }
  const res = await fetch(BASE + path, opts)
  if (!res.ok) {
    const text = await res.text().catch(() => '')
    throw new Error(`${method} ${path} → ${res.status}: ${text}`)
  }
  // 204 No Content or empty body
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
  upload:          (fd)      => fetch(BASE + '/api/upload', { method: 'POST', body: fd }).then(r => r.json()),
  updateType: (id, type) => request('PATCH', `/api/reviews/${id}`, { type }),
}
