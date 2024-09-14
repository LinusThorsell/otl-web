<script>
  import EventCard from '$lib/EventCard.svelte';
  import TourResultsTable from '$lib/TourResultsTable.svelte';

  export let data;
</script>

<div class="flex flex-col justify-center items-center">
  {#each data.tours as tour}
    <EventCard event={tour} button="Visa resultat" url={'/results/' + tour.id} />
  {/each}
</div>
