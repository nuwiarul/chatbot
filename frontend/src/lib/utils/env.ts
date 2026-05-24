type PublicEnv = {
  backendUrl: string;
  apiKey: string;
};

function required(name: string): string {
  const value = import.meta.env[name] as string | undefined;
  if (!value || value.trim().length === 0) throw new Error(`Missing env: ${name}`);
  return value;
}

export function env(): PublicEnv {
  return {
    backendUrl: required('VITE_BACKEND_URL'),
    apiKey: required('VITE_API_KEY')
  };
}

