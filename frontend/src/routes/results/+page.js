import { PUBLIC_BACKEND_URL } from '$env/static/public';
export async function load({ fetch }) {
  let res = await fetch(PUBLIC_BACKEND_URL + '/api/tours')
  let tours = await res.json()

  return {
    tours: tours.body
  };
}
