export async function load({ fetch }) {
  let res = await fetch(`http://backend:8000/api/tours`)
  let tours = await res.json()

  return {
    tours: tours.body
  };
}
