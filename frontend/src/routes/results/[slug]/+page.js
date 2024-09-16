export async function load({ params, fetch }) {
  let res = await fetch(`https://api.teamontheline.com/api/tour/leaderboard/${params.slug}`)
  let tour = await res.json()

  return {
    tourLeaderboard: tour.body
  };
}
