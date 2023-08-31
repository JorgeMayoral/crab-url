import { ApiResponse } from "../interfaces";

export async function addUrl(url: string): Promise<ApiResponse> {
  try {
    const res = await fetch("/add", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ url }),
    });
    const { data } = await res.json();
    if (res.ok) {
      return { data };
    }
    return { error: data.error };
  } catch (err) {
    return { error: (err as Error).message };
  }
}