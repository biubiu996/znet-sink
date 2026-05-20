<script lang="ts">
  export let size: 'sm' | 'md' | 'lg' = 'md';
  export let color: 'default' | 'primary' | 'white' = 'default';

  const sizeClasses = {
    sm: 'w-3 h-3',
    md: 'w-4 h-4',
    lg: 'w-6 h-6',
  };

  const colorClasses = {
    default: 'border-muted-foreground/30 border-t-muted-foreground',
    primary: 'border-primary/30 border-t-primary',
    white: 'border-white/30 border-t-white',
  };
</script>

<div 
  class={`${sizeClasses[size]} ${colorClasses[color]} rounded-full border-2 animate-spin`}
  aria-label="加载中"
></div>
