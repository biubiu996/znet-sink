<script lang="ts">
  /**
   * Skeleton — animated placeholder for loading content.
   *
   * Props:
   * - width:  CSS width (e.g. "100%", "200px", "3rem")
   * - height: CSS height
   * - radius: border-radius override; defaults to var(--radius-sm)
   * - class:  additional CSS classes
   */
  export let width = '100%';
  export let height = '14px';
  export let radius = 'var(--radius-sm, 6px)';
  export let className = '';
</script>

<div
  class={`skeleton ${className}`}
  style="width: {width}; height: {height}; border-radius: {radius};"
  aria-hidden="true"
></div>
