import { useMutation, useQuery } from '@tanstack/react-query';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { load } from '@tauri-apps/plugin-store';
import { SaveIcon } from 'lucide-react';
import { useEffect, useId } from 'react';
import { z } from 'zod/v4-mini';

import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { FieldGroup } from '@/components/ui/field';
import { LoadingButton } from '@/components/ui/loading-button';
import { Spinner } from '@/components/ui/spinner';
import { useAppForm } from '@/lib/use-app-form';

const settingsSchema = z.object({
  url: z.url(),
  token: z.string().check(z.minLength(1)),
});
type SettingsType = z.infer<typeof settingsSchema>;

export default function SettingsForm() {
  const { data, isLoading } = useQuery({
    queryKey: ['settings'],
    queryFn: async () => {
      const store = await load('store.json');
      const settings = await store.get<SettingsType>('settings');
      return settings ?? { url: '', token: '' };
    },
  });

  const { mutateAsync, isPending } = useMutation({
    mutationFn: async (payload: SettingsType) => {
      const store = await load('store.json');
      await store.set('settings', payload);
    },
    onSuccess: () => getCurrentWindow().close(),
  });

  const form = useAppForm({
    defaultValues: {
      url: data?.url ?? '',
      token: data?.token ?? '',
    },
    validators: {
      onSubmit: settingsSchema,
    },
    onSubmit: async ({ value }) => {
      const parsed = settingsSchema.parse(value);
      await mutateAsync(parsed);
    },
  });
  const id = useId();

  useEffect(() => {
    getCurrentWindow().show();
  }, []);

  if (isLoading) {
    return (
      <main className="flex h-dvh w-dvw flex-col">
        <Spinner className="m-auto" />
      </main>
    );
  }

  return (
    <main className="p-4">
      <Card>
        <CardHeader>
          <CardTitle>Settings</CardTitle>
        </CardHeader>
        <CardContent>
          <form
            id={id}
            onSubmit={(e) => {
              e.preventDefault();
              form.handleSubmit();
            }}
          >
            <FieldGroup>
              <form.AppField name="url">
                {(field) => <field.InputField label="URL" type="url" required />}
              </form.AppField>
              <form.AppField name="token">
                {(field) => <field.InputField label="Token" type="password" required />}
              </form.AppField>
            </FieldGroup>
          </form>
        </CardContent>
        <CardFooter className="justify-end">
          <LoadingButton form={id} type="submit" isLoading={isPending} icon={<SaveIcon />}>
            Save
          </LoadingButton>
        </CardFooter>
      </Card>
    </main>
  );
}
