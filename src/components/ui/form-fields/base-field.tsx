import { FieldApi } from '@tanstack/react-form';
import { ReactNode, useId } from 'react';

import {
  Field,
  FieldContent,
  FieldDescription,
  FieldError,
  FieldLabel,
} from '@/components/ui/field';
import { useFieldContext } from '@/lib/app-form-context';
import { type ClassNamesType, cn } from '@/lib/utils';

export type BasicFieldProps = {
  label?: string;
  description?: string;
  required?: boolean;
  classNames?: ClassNamesType;
};

type PropsWithId = {
  id?: string;
};

export type WithFieldProps<T extends PropsWithId = PropsWithId> = T & BasicFieldProps;

type FieldInputParams<TData> = {
  id: string;
  field: FieldApi<
    any,
    string,
    TData,
    any,
    any,
    any,
    any,
    any,
    any,
    any,
    any,
    any,
    any,
    any,
    any,
    any,
    any,
    any,
    any,
    any,
    any,
    any,
    any
  >;
  isInvalid: boolean;
  className?: string;
};

type BaseFieldProps<TData> = BasicFieldProps & {
  id?: string;
  children: (params: FieldInputParams<TData>) => ReactNode;
};

export function BaseField<TData = string>(props: BaseFieldProps<TData>) {
  const uid = useId();
  const { label, description, required, id = uid, children, classNames, ...rest } = props;
  const field = useFieldContext<TData>();

  const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid;

  return (
    <Field className={cn(classNames?.root)} data-invalid={isInvalid} {...rest}>
      {label && (
        <FieldLabel className={cn('text-nowrap', { required })} htmlFor={id}>
          {label}
        </FieldLabel>
      )}
      <FieldContent>
        {children({ id, field, isInvalid, className: cn(classNames?.content) })}
        <FieldError errors={field.state.meta.errors} />
        {description && <FieldDescription>{description}</FieldDescription>}
      </FieldContent>
    </Field>
  );
}
