<script>
  import { Footer, FooterLinkGroup, FooterLink, FooterBrand, FooterIcon, FooterCopyright } from 'flowbite-svelte';
  import { FacebookSolid, InstagramSolid } from 'flowbite-svelte-icons';
</script>

<Footer footerType="sitemap">
  <div class="grid grid-cols-2 gap-8 py-8 px-6 md:grid-cols-4">
    <div>
      <h2 class="mb-6 text-sm font-semibold text-gray-400 uppercase">Vi är</h2>
      <FooterLinkGroup ulClass="text-gray-300">
        <FooterLink liClass="mb-4" href="/about">Linus Thorsell</FooterLink>
        <FooterLink liClass="mb-4" href="/about">Vincent Müller</FooterLink>
        <FooterLink liClass="mb-4" href="/about">Robert Sjöblom</FooterLink>
        <FooterLink liClass="mb-4" href="/about">Oscar Lövborg</FooterLink>
      </FooterLinkGroup>
    </div>
    <div>
      <h2 class="mb-6 text-sm font-semibold uppercase text-gray-400">Företaget</h2>
      <FooterLinkGroup ulClass="text-gray-300">
        <p class="mb-4">OffTheLine AB</p>
        <p>Org. Nummer: 559427-4101</p>
      </FooterLinkGroup>
    </div>
  </div>
  <div class="py-6 px-4 bg-gray-700 md:flex md:items-center md:justify-between">
    <p class="text-sm text-gray-400 sm:text-center">OffTheLine AB - <a href="mailto:kontakta@teamontheline.com">kontakta@teamontheline.com</a></p>
    <div class="flex mt-4 space-x-6 rtl:space-x-reverse sm:justify-center md:mt-0">
      <FooterIcon href="https://www.facebook.com/TeamOnTheLine">
        <FacebookSolid class="w-5 h-5 text-gray-400 dark:text-gray-500 hover:text-gray-900 dark:hover:text-white" />
      </FooterIcon>
      <FooterIcon href="https://www.instagram.com/ontheline_tour">
        <InstagramSolid class="w-5 h-5 text-gray-400 dark:text-gray-500 hover:text-gray-900 dark:hover:text-white" />
      </FooterIcon>
    </div>
  </div>
</Footer>
