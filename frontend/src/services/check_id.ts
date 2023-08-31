import { ApiResponse } from "../interfaces";

export async function checkId(id: string): Promise<ApiResponse> {
  try {
    const res = await fetch("/check", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ id }),
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