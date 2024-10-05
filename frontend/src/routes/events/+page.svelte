<script>
  import { Button } from 'flowbite-svelte';
  import { ArrowRightOutline } from 'flowbite-svelte-icons';
  import EventCarousel from '$lib/EventCarousel.svelte';
  import Container from '$lib/Container.svelte';

  export let data;
</script>

<Container>
  <EventCarousel events={data.events} />
</Container>
