# Revi — User Guide

This guide covers everything a human reviewer needs to know: setting up the workspace, navigating the UI, leaving anchored comments, resolving feedback, and using the archive.

If you want the styled web version, open [the live guide](https://yorha-agents.github.io/Revi/guide/).

---

## Workspace Setup

Revi auto-discovers files from the `backend/workspace/` directory. Drop files into the appropriate subfolder and they will appear in the sidebar automatically on next server start (or immediately if using the upload API).

### Folder structure

```
backend/workspace/
  plans/          ← Markdown documents (.md)
  designs/        ← Image files (.png, .jpg, .gif, .webp, .svg)
  prototypes/     ← Interactive HTML files (.html)
```

### What goes where

**plans/** — Text-based planning documents in Markdown format. These are rendered with full formatting including headings, code blocks, and tables. The heading hierarchy is used to build the collapsible document index. Use this for architecture documents, sprint plans, technical specs, and similar written artefacts.

**designs/** — Static image files showing UI mockups, wireframes, or diagrams. Revi displays these on a dark canvas with clickable numbered annotation pins. Each pin maps to an entry in the legend below the canvas, and clicking a pin opens the comment form pre-filled with that annotation reference.

**prototypes/** — Self-contained interactive HTML files. Revi wraps them in a step-by-step viewer with a left-side navigation panel. Each step has a comment button, and the right pane shows the visual at that step. This works well for walkthroughs or click-through flows.

### Naming conventions

File names become the item's display title via title-casing, so use descriptive names:

- `sprint-1-design.md` → "Sprint 1 Design"
- `ui-mockup-v2.png` → "UI Mockup V2"

---

## Uploading Files

### Via the API

```bash
curl -X POST http://localhost:8000/api/upload/plans \
  -F "file=@my-plan.md"
```

Use `plans`, `designs`, or `prototypes` as the subfolder in the path.

### Via drag-and-drop (future)

The upload endpoint is designed to support a drag-and-drop UI — place files into the correct subfolder of `workspace/` directly as an alternative.

---

## Navigating the Interface

### Sidebar

The left sidebar lists all workspace items grouped by type (Plans, Designs, Prototypes). Each group is collapsible. Open comment counts appear as red badges next to item names.

Use the search box at the top of the sidebar to filter items by name.

### Home dashboard

The home screen shows aggregate stats: total items, open comments, resolved comments, and archived batches. Each item card shows its type, description, and comment status. Click **Open →** to enter the review view.

### Language toggle

Click **EN** or **中文** in the top-right corner to switch the UI language. Bilingual items will display their translated titles and descriptions.

---

## Reviewing an Item

### Opening an item

Click **Open →** on a card, or click the item name in the sidebar. This opens the review view with a split layout:

- **Left (optional):** Document index panel (plans only)
- **Center:** Content viewer (markdown / design / prototype)
- **Right:** Comment panel

### Document index (plans only)

Click the **☰ Index** button in the header to open the collapsible table of contents. It lists all headings (h1–h3) in the document. Click any heading to smooth-scroll to it. The active heading is highlighted as you scroll.

### In-page search (plans only)

Click the **⌕ Search** button to open the search bar at the top of the markdown viewer. Type to highlight matches; use **↑ ↓** or **Enter** to navigate between them. Press **Esc** or the **×** button to close.

### Design viewer

The design viewer shows all frames on a dark canvas. Double-click a frame to expand it. Numbered yellow pins mark annotation points — click a pin to open the comment form pre-filled with that annotation reference. Hover over a comment card in the right panel to highlight the corresponding pin.

### Prototype viewer

The prototype viewer shows a step-by-step walkthrough. Use the left navigation panel to jump between steps, or use the **Back / Next →** buttons. Each step has a **💬** button to leave a comment anchored to that step. Zooming and panning are supported in the visual area.

---

## Leaving a Comment

### Opening the comment form

Click **Add Comment** in the comment panel (top-right of the review view). The form opens below the action bar.

### Pre-filled anchors

Several interactions pre-fill the anchor field automatically:

| Action | Resulting anchor |
|--------|-----------------|
| Select text in a plan, click **Comment on selection** tooltip | `quote` anchor with the selected text |
| Click an annotation pin in a design | `annotation` anchor with the pin number |
| Click **💬** on a prototype step | `step` anchor with the step index |

When an anchor is pre-filled, it appears as a coloured tag at the top of the form. Click **×** to remove it if you want a general comment instead.

### Manual reference

If no anchor was pre-filled, you can type a reference in the **Reference** field. For example, type `## Architecture` to link the comment to that heading. This creates a `section` reference.

Leave the reference field empty for a general comment (no specific anchor).

### Submitting

Fill in **Author** (your name) and **Comment**, then click **Submit**. The comment appears immediately in the **Open** tab.

---

## Comment Anchor Types

### section

Links to a markdown heading. The comment card shows the heading name, and hovering over the card scrolls the markdown viewer to that heading and flashes it.

**Example reference:** `{ "type": "section", "value": "## Storage Layer" }`

### quote

Links to a specific text selection within a plan. The full quoted text is shown below the comment author. Hovering the card highlights the quoted text inline in the document.

**Example reference:** `{ "type": "quote", "value": "Comments are stored as JSON files", "section": "Storage Layer" }`

The `section` sub-field is the heading the selection was under — used to disambiguate if the same text appears multiple times.

### line

Links to a specific line number in the document.

**Example reference:** `{ "type": "line", "value": 42 }`

### annotation

Links to a numbered annotation pin on a design image. The comment card shows the pin number. Hovering the card highlights the corresponding pin in the design viewer.

**Example reference:** `{ "type": "annotation", "value": "3", "label": "Comment Panel" }`

The `label` field is the annotation legend title, e.g. "Comment Panel".

### step

Links to a step in an interactive prototype. Step indices are 0-based internally but displayed as 1-based in the UI (step index 1 shows as "Step 2").

**Example reference:** `{ "type": "step", "value": 1, "label": "Open for Review" }`

### general

No specific anchor — the comment applies to the item as a whole.

**Example reference:** `{ "type": "general", "value": null }`

---

## Resolving Comments

When a comment has been addressed, click **✓ Mark resolved** on the comment card. The comment moves from the **Open** tab to the **Resolved** tab.

The active section indicator highlights comment cards that are related to the section currently visible in the markdown viewer — these section-matched cards glow with a blue ring.

---

## Archive Workflow

### Why archive?

Resolved comments stay in the Resolved tab until explicitly archived. Archiving bundles all resolved comments into a timestamped batch and removes them from the active list, keeping the workspace clean for the next review round.

### How to archive

When there are resolved comments, the **Archive resolved (N)** button appears in the action bar. Click it to create a new archive batch.

The tab resets to **Open** and the Resolved tab shows 0. The archived comments are preserved in the Archive view.

### Viewing archived batches

Click **Archive** in the bottom of the left sidebar (or navigate to `/archive`). The archive view lists all items that have archived batches, grouped by item. Each batch shows:

- Batch number (newest first)
- Timestamp of when it was archived
- Comment count
- Each archived comment with its author, resolved date, reference, and content

---

## Agent Export Preview

At the bottom of the comment panel, expand **Agent Export Preview** to see the structured JSON that AI agents receive when they call `GET /api/export/{item_id}`. This is useful for verifying that agents will see the right information before handing off.

The export includes only open comments — resolved and archived comments are omitted. This is by design: resolved means done, and agents should only act on open feedback.

---

## Tips

- **Comment counts** appear as red badges in the sidebar. A badge of 0 means no open feedback — the item is clean.
- **Hover a comment card** to highlight its anchor in the content viewer (works for quotes, sections, annotation pins, and prototype steps).
- **Scroll the plan** while viewing — the comment panel auto-highlights cards matching the section currently in view.
- **Archive frequently** to keep Resolved tabs clean. Archived batches are permanent and browsable; nothing is deleted.
