import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

import '@/App.css';
import SettingsForm from '@/components/settings-form';

const queryClient = new QueryClient();

export default function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <SettingsForm />
    </QueryClientProvider>
  );
}
