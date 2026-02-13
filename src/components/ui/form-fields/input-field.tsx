import type { ComponentProps } from 'react';

import { Input } from '@/components/ui/input';

import { BaseField, type WithFieldProps } from './base-field';

type InputFieldProps = WithFieldProps<ComponentProps<'input'>> & {
  submitOnEnter?: boolean;
};

export default function InputField(props: InputFieldProps) {
  const { label, description, id, classNames, submitOnEnter, ...rest } = props;

  return (
    <BaseField {...{ label, description, id, classNames }}>
      {({ id, field, isInvalid, className }) => (
        <Input
          id={id}
          className={className}
          name={field.name}
          value={field.state.value ?? ''}
          onBlur={field.handleBlur}
          onChange={(e) => field.handleChange(e.target.value)}
          aria-invalid={isInvalid}
          data-testid={field.name}
          onKeyDown={
            submitOnEnter
              ? (e) => {
                  if (e.key === 'Enter' || e.key === 'Return') {
                    e.preventDefault();
                    field.form.handleSubmit();
                  }
                }
              : undefined
          }
          {...rest}
        />
      )}
    </BaseField>
  );
}
