<script>
  import { Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell, TableSearch } from 'flowbite-svelte';

  export let tour;
  export let entries;
</script>

<Table hoverable={true}>
  <TableHead>
    <TableHeadCell>Plats</TableHeadCell>
    <TableHeadCell>Namn</TableHeadCell>
    <TableHeadCell>PDGA</TableHeadCell>
    <TableHeadCell>Bästa {tour.score_count}</TableHeadCell>
    {#each Array(tour.score_count) as _, index (index)}
      <TableHeadCell>{index + 1}</TableHeadCell>
    {/each}
  </TableHead>
  <TableBody tableBodyClass="divide-y">
    {#each entries as entry}
      <TableBodyRow>
        <TableBodyCell>{entry.placement}</TableBodyCell>
        <TableBodyCell>{entry.user.firstname} {entry.user.lastname}</TableBodyCell>
        <TableBodyCell>{entry.user.pdga ? entry.user.pdga : '-'}</TableBodyCell>
        <TableBodyCell>{entry.total_score}</TableBodyCell>
        {#each Array(tour.score_count) as _, index (index)}
          <TableBodyCell>{entry.scores[index] ? entry.scores[index].score : '-'}</TableBodyCell>
        {/each}
      </TableBodyRow>
    {/each}
  </TableBody>
</Table>

