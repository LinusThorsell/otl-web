<script>
  import { Timeline, TimelineItem, Button } from 'flowbite-svelte';
  import { ArrowRightOutline } from 'flowbite-svelte-icons';

  import PersonalCard from '$lib/PersonalCard.svelte';
</script>

<div class="flex flex-col justify-center items-center">
  <h1 class="text-3xl font-bold mb-4 pt-5 text-center">Vilka är vi?</h1>
    <div class="mb-10 flex flex-wrap w-3/4 justify-around">
      <PersonalCard firstname="Linus" lastname="Thorsell" description="Hemsidan & Företaget" image="https://www.pdga.com/files/styles/large/public/pictures/picture-1959196-1649747166.jpg?itok=XSIJxB6Z" />
      <PersonalCard firstname="Robert" lastname="Sjöblom" description="Sponsring & Marknad" image="https://i.imgur.com/nzu8afm.jpeg" />
      <PersonalCard firstname="Vincent" lastname="Müller" description="Grafik & Media" image="https://www.pdga.com/files/styles/large/public/pictures/picture-2078131-1685723270.jpg?itok=O2ludU1o" />
      <PersonalCard firstname="Oscar" lastname="Lövborg" description="Planering & Utskick" image="https://www.pdga.com/files/styles/large/public/pictures/picture-2070071-1703256637.jpg?itok=vgfEk2Ug" />
    </div>
  <!-- <h1 class="mb-10 text-3xl font-bold text-center">Hur hamnade vi här?</h1> -->
  <!-- <Timeline> -->
  <!--   <TimelineItem title="Första evenemanget går av stapeln" date="Januari 2022"> -->
  <!--     <p class="mb-4 text-base font-normal text-gray-500 dark:text-gray-400">Linus & Vincent kör det första OnTheLine Tour eventet</p> -->
  <!--   </TimelineItem> -->
  <!--   <TimelineItem title="Application UI code in Tailwind CSS" date="March 2022"> -->
  <!--     <p class="text-base font-normal text-gray-500 dark:text-gray-400">All of the pages and components are first designed in Figma and we keep a parity between the two versions even as we update the project.</p> -->
  <!--   </TimelineItem> -->
  <!--   <TimelineItem title="Application UI code in Tailwind CSS" date="April 2022"> -->
  <!--     <p class="text-base font-normal text-gray-500 dark:text-gray-400">Get started with dozens of web components and interactive elements built on top of Tailwind CSS.</p> -->
  <!--   </TimelineItem> -->
  <!-- </Timeline> -->
</div>
