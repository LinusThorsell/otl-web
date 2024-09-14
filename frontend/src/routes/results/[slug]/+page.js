export async function load({ params, fetch }) {
  let res = await fetch(`http://backend:8000/api/tour/leaderboard/${params.slug}`)
  let tour = await res.json()

  return {
    tourLeaderboard: tour.body
  };
}
