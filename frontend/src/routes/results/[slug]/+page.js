import { PUBLIC_BACKEND_URL } from '$env/static/public';
export async function load({ params, fetch }) {
  let res = await fetch(PUBLIC_BACKEND_URL + `/api/tour/leaderboard/${params.slug}`)
  let tour = await res.json()

  return {
    tourLeaderboard: tour.body
  };
}
