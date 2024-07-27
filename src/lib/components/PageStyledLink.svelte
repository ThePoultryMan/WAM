<script lang="ts">

  export let href: string;
  let clazz: string;
  export { clazz as class };
  export let active;
</script>

<a {href} class={active ? clazz : ""}>
  <slot />
</a>
