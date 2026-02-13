import { createFormHook } from '@tanstack/react-form';
import { lazy } from 'react';

import { LoadingButton, type LoadingButtonProps } from '@/components/ui/loading-button';

import { fieldContext, formContext, useFormContext } from './app-form-context';

const InputField = lazy(() => import('@/components/ui/form-fields/input-field'));

type SubmitButtonProps = LoadingButtonProps;

function SubmitButton({ isLoading, ...rest }: SubmitButtonProps) {
  const form = useFormContext();

  return (
    <form.Subscribe selector={(state) => state.isSubmitting}>
      {(isSubmitting) => (
        <LoadingButton type="submit" isLoading={isSubmitting || isLoading} {...rest} />
      )}
    </form.Subscribe>
  );
}

export const { useAppForm, withForm, withFieldGroup } = createFormHook({
  fieldComponents: {
    InputField,
  },
  formComponents: {
    SubmitButton,
  },
  fieldContext,
  formContext,
});
