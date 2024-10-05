<script>
  import Container from '$lib/Container.svelte';
  import EventCard from '$lib/EventCard.svelte';

  export let data;
</script>

<Container>
  <div class="flex">
    {#each data.tours as tour}
      <EventCard event={tour} button="Visa resultat" url={'/results/' + tour.id} />
    {/each}
  </div>
</Container>
