<script>
  import { Navbar, NavBrand, NavLi, NavUl, NavHamburger, DarkMode } from 'flowbite-svelte'
  import { onMount } from 'svelte';

  let isDark = false;
  onMount(() => {
    isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
  })
</script>

<Navbar let:hidden let:toggle>
  <NavBrand href="/">
    <img
      src={isDark ? '/logo_dark.png' : '/logo.png'}
      class="mr-3 h-6 sm:h-9"
      alt="OnTheLine Logo"
    />
  </NavBrand>
  <NavHamburger on:click={toggle} />
  <NavUl {hidden}>
    <NavLi href="/" active={true}>Hem</NavLi>
    <NavLi href="/events">Event</NavLi>
    <NavLi href="/results">Resultat</NavLi>
    <NavLi href="/media">Media</NavLi>
    <NavLi href="/about">Om oss</NavLi>
    <NavLi href="/contact">Kontakt</NavLi>
  </NavUl>
</Navbar>
