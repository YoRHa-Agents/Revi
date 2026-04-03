import { reactive } from 'vue'
import { api } from '../api/client.js'

export const state = reactive({
  items: [],
  comments: {},   // { [itemId]: Comment[] }
  archive: {},    // { [itemId]: ArchiveBatch[] }
  loading: false,
  error: null,

  // Workspace / config state
  config: null,
  configLoading: false,
  workspaceConfigured: null, // null = unknown, true/false from server

  getItem(id) {
    return this.items.find(i => i.id === id)
  },
  getComments(itemId) {
    return this.comments[itemId] || []
  },
  openCount(itemId) {
    return (this.comments[itemId] || []).filter(c => c.status === 'open').length
  },
  resolvedCount(itemId) {
    return (this.comments[itemId] || []).filter(c => c.status === 'resolved').length
  },

  async fetchConfig() {
    this.configLoading = true
    try {
      this.config = await api.getConfig()
      this.workspaceConfigured = this.config.workspaceConfigured
    } catch (e) {
      this.error = e.message
      this.workspaceConfigured = false
    } finally {
      this.configLoading = false
    }
  },

  async setWorkspacePath(path) {
    this.configLoading = true
    this.error = null
    try {
      this.config = await api.updateConfig({ workspacePath: path })
      this.workspaceConfigured = this.config.workspaceConfigured
      await this.fetchItems()
    } catch (e) {
      this.error = e.message
      throw e
    } finally {
      this.configLoading = false
    }
  },

  async fetchItems() {
    this.loading = true
    this.error = null
    try {
      this.items = await api.getReviews()
    } catch (e) {
      this.error = e.message
    } finally {
      this.loading = false
    }
  },

  async fetchComments(itemId) {
    try {
      this.comments[itemId] = await api.getComments(itemId)
    } catch (e) {
      this.error = e.message
    }
  },

  async addComment(itemId, { author, content, reference }) {
    const comment = await api.addComment(itemId, { author, content, reference })
    if (!this.comments[itemId]) this.comments[itemId] = []
    this.comments[itemId].push(comment)
    return comment
  },

  async resolveComment(itemId, commentId) {
    const updated = await api.resolveComment(itemId, commentId)
    const idx = (this.comments[itemId] || []).findIndex(c => c.id === commentId)
    if (idx !== -1) this.comments[itemId][idx] = updated
    return updated
  },

  async archiveResolved(itemId) {
    const batch = await api.archiveResolved(itemId)
    if (this.comments[itemId]) {
      this.comments[itemId] = this.comments[itemId].filter(c => c.status === 'open')
    }
    if (!this.archive[itemId]) this.archive[itemId] = []
    this.archive[itemId].unshift(batch)
    return batch
  },

  async fetchArchive(itemId) {
    try {
      this.archive[itemId] = await api.getArchive(itemId)
    } catch (e) {
      this.error = e.message
    }
  },

  async exportForAgent(itemId) {
    return api.exportForAgent(itemId)
  },

  async updateItemType(itemId, type) {
    const updated = await api.updateType(itemId, type)
    const idx = this.items.findIndex(i => i.id === itemId)
    if (idx !== -1 && updated) this.items[idx] = { ...this.items[idx], type: updated.type }
  },
})
