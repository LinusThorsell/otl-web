<script>
  import { Accordion, AccordionItem } from 'flowbite-svelte';

  import CoverImage from '$lib/CoverImage.svelte';
  import TourResultsTable from '$lib/TourResultsTable.svelte';

  export let data;
</script>

<div class="flex flex-col justify-center items-center mt-6">
  <h1 class="sm:text-3xl text-2xl font-bold text-center">Resultat för {data.tourLeaderboard.tour.title}</h1>
  <h2 class="text-lg text-center">De {data.tourLeaderboard.tour.score_count} bästa poängen räknas!</h2>
  <Accordion flush class="mt-3 sm:w-1/2 w-11/12 mt-4" activeClass="bg-blue-100 dark:bg-gray-800 text-blue-600 dark:text-white focus:ring-4 focus:ring-blue-200 dark:focus:ring-blue-800" inactiveClass="text-gray-500 dark:text-gray-400 hover:bg-gray-700 dark:hover:bg-gray-800">
    {#each Object.entries(data.tourLeaderboard.divisions) as division}
      <AccordionItem class="w-full">
        <span slot="header">{division[0]}</span>
        <TourResultsTable tour={data.tourLeaderboard.tour} entries={division[1].entries} />
      </AccordionItem>
    {/each}
  </Accordion>
</div>
