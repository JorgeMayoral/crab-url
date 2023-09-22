import {
	Table,
	TableBody,
	TableCell,
	TableColumn,
	TableHeader,
	TableRow,
} from '@nextui-org/react';
import { flag } from 'country-emoji';

interface Props {
	countries: {
		[key: string]: number;
	};
}

export function CountryVisitsTable({ countries }: Props) {
	return (
		<Table hideHeader isStriped>
			<TableHeader>
				<TableColumn>Country</TableColumn>
				<TableColumn>Visits</TableColumn>
			</TableHeader>
			<TableBody>
				{Object.keys(countries).map((country) => (
					<TableRow key={country}>
						<TableCell className="text-lg">
							{generateCountryString(country)}
						</TableCell>
						<TableCell className="text-xl font-bold">
							{countries[country]}
						</TableCell>
					</TableRow>
				))}
			</TableBody>
		</Table>
	);
}

function generateCountryString(country: string): string {
	const countryFlag = flag(country) ?? '🏴‍☠️';
	return `${countryFlag} ${country}`;
}
