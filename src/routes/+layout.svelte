<script>
  import { page } from "$app/stores";

  import "../app.css";

  import PageStyledLink from "$components/PageStyledLink.svelte";
</script>

<div class="flex">
  <div class="leading-none *:block *:pl-3 *:pr-4 *:py-1.5">
    <PageStyledLink href="/" class="bg-blue-400" active={$page.url.pathname === "/"}>Play</PageStyledLink>
    <PageStyledLink href="/info" class="bg-blue-400" active={$page.url.pathname === "/info"}>Info</PageStyledLink>
    <PageStyledLink href="/addons" class="bg-blue-400" active={$page.url.pathname.includes("/addons")}>Addons</PageStyledLink>
  </div>
  <div class="mx-5 mt-2 w-full">
    <slot></slot>
  </div>
</div>
