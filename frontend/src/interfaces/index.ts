export interface ShortedUrl {
  id: string;
  target: string;
  expire_in: number;
}

export type AppError = string;

export interface ApiResponse {
  data?: ShortedUrl
  error?: AppError
}

export interface Metrics {
  totalVisits: number;
  countries: {
    [key: string]: number;
  }
}
