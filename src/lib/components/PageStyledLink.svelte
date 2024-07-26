<script lang="ts">
  import { page } from "$app/stores";

  export let href: string;
  let clazz: string;
  export { clazz as class };
</script>

<a {href} class={$page.url.pathname.includes(href) ? clazz : ""}>
  <slot />
</a>
