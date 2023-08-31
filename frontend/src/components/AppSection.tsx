import { Button, Input } from '@nextui-org/react';
import { ShortedUrl } from '../interfaces';
import { ShortedUrlCard } from './ShortedUrlCard';
import { DOMAIN } from '../constants';

interface Props {
	title: string;
	inputType: 'url' | 'text';
	placeholder: string;
	label: string;
	inputValue: string;
	onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
	buttonText: string;
	onClick: () => void;
	shortedUrlData: ShortedUrl | null;
	loading: boolean;
	error: string;
	validState: 'valid' | 'invalid';
	sectionType: 'short' | 'check';
}

export function AppSection({
	title,
	placeholder,
	label,
	inputType,
	inputValue,
	onChange,
	onClick,
	buttonText,
	shortedUrlData,
	validState,
	error,
	loading,
	sectionType,
}: Props) {
	return (
		<section className="flex flex-col gap-4 w-full items-center">
			<h1 className="text-xl font-bold">{title}</h1>
			{sectionType === 'check' && (
				<p>
					ID example: https://{DOMAIN}/r/
					<span className="underline font-bold text-blue-500">Ys-43j</span>
				</p>
			)}
			<Input
				type={inputType}
				label={label}
				placeholder={placeholder}
				value={inputValue}
				onChange={onChange}
				errorMessage={error}
				validationState={validState}
			/>
			<Button
				color="primary"
				onClick={onClick}
				isLoading={loading}
				isDisabled={inputValue.length === 0 || error.length > 0}
				size="lg"
			>
				{buttonText}
			</Button>
			{shortedUrlData && <ShortedUrlCard data={shortedUrlData} />}
		</section>
	);
}
