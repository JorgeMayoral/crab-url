import { Metrics } from "../interfaces";

interface MetricsApiResponse {
  metrics: {
    url_id: string;
    country: string;
    date: string;
  }[]
}

const EMPTY_METRICS: Metrics = {
  totalVisits: 0,
  countries: {}
};

export async function getUrlMetrics(url: string): Promise<Metrics> {
  const endpoint = `/metrics?url_id=${url}`;
  try {
    const res = await fetch(endpoint);
    const { metrics }: MetricsApiResponse = await res.json();
    if (res.ok) {
      const totalVisits = metrics.length;
      let countries = metrics.reduce((acc, curr) => {
        if (acc[curr.country]) {
          acc[curr.country] += 1;
        } else {
          acc[curr.country] = 1;
        }
        return acc;
      }, {} as { [key: string]: number });
      const shortedCountries = Object.entries(countries).sort((a, b) => b[1] - a[1]);
      countries = shortedCountries.reduce((acc, curr) => {
        acc[curr[0]] = curr[1];
        return acc;
      }, {} as { [key: string]: number });
      return {
        totalVisits,
        countries,
      };
    }
    return EMPTY_METRICS;
  } catch (err) {
    console.error(err);
    return EMPTY_METRICS;
  }
}