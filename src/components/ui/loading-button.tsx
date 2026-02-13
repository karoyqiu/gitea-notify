import type { ReactNode } from 'react';

import { Button, type ButtonProps } from './button';
import { Spinner } from './spinner';

export type LoadingButtonProps = ButtonProps & { icon?: ReactNode; isLoading?: boolean };

export function LoadingButton({ icon, isLoading, children, ...props }: LoadingButtonProps) {
  return (
    <Button disabled={isLoading} {...props}>
      {isLoading ? <Spinner /> : icon}
      {children}
    </Button>
  );
}
