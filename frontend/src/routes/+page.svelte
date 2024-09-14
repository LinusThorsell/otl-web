<script>
  import LandingImage from '$lib/LandingImage.svelte';
  import EventCarousel from '$lib/EventCarousel.svelte';

  export let data;
</script>

<div>
  <LandingImage>
    <EventCarousel events={data.events} eventsPerSlide={3} />
  </LandingImage>
</div>
