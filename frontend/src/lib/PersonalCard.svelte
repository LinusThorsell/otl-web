<script>
  export let firstname;
  export let lastname;
  export let image;
  export let description;
</script>

<div class="p-4 flex flex-col justify-center items-center">
  <img
    class="w-64 h-64 rounded-full object-cover"
    src={image}
    alt={firstname + " " + lastname}
  />
  <h1 class="mt-3 text-3xl font-bold text-center">{firstname} {lastname}</h1>
  <h1 class="text-xl text-center">{description}</h1>
</div>
