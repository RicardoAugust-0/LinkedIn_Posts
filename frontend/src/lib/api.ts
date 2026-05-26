const rawUrl = (import.meta.env.VITE_API_URL as string) || 'http://localhost:3000';
export const API_URL = rawUrl.endsWith('/') ? rawUrl.slice(0, -1) : rawUrl;

