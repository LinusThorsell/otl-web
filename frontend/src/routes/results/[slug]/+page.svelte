<script>
  import { Accordion, AccordionItem } from 'flowbite-svelte';

  import CoverImage from '$lib/CoverImage.svelte';
  import TourResultsTable from '$lib/TourResultsTable.svelte';

  export let data;

  const divisionOrder = ['MPO', 'FPO', 'MA1', 'MA2', 'MA3', 'MA4', 'MJ15'];
  function sortDivisions(divisions) {
    return Object.entries(divisions).sort(([keyA], [keyB]) => {
      const indexA = divisionOrder.indexOf(keyA);
      const indexB = divisionOrder.indexOf(keyB);

      // If both keys are in the predefined order, sort them according to that order
      if (indexA !== -1 && indexB !== -1) {
        return indexA - indexB;
      }

      // If only one of the keys is in the predefined order, prioritize it
      if (indexA !== -1) return -1;
      if (indexB !== -1) return 1;

      // If neither key is in the predefined order, retain their original order
      return 0;
    });
  }
</script>

<div class="flex flex-col justify-center items-center mt-6">
  <h1 class="sm:text-3xl text-2xl font-bold text-center">Resultat för {data.tourLeaderboard.tour.title}</h1>
  <h2 class="text-lg text-center">De {data.tourLeaderboard.tour.score_count} bästa poängen räknas!</h2>
  <Accordion flush class="mt-3 lg:w-1/2 w-11/12 mt-4" activeClass="bg-blue-100 dark:bg-gray-800 text-blue-600 dark:text-white focus:ring-4 focus:ring-blue-200 dark:focus:ring-blue-800" inactiveClass="text-gray-500 dark:text-gray-400 hover:bg-gray-700 dark:hover:bg-gray-800">
    {#each sortDivisions(data.tourLeaderboard.divisions) as division}
      <AccordionItem class="w-full">
        <span slot="header">{division[0]}</span>
        <TourResultsTable tour={data.tourLeaderboard.tour} entries={division[1].entries} />
      </AccordionItem>
    {/each}
  </Accordion>
</div>
