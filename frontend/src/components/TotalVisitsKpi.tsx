import { Card, CardBody, CardHeader } from '@nextui-org/react';

interface Props {
	visits: number;
}

export function TotalVisitsKpi({ visits }: Props) {
	return (
		<Card>
			<CardHeader>
				<h3 className="text-lg font-bold">Total Visits</h3>
			</CardHeader>
			<CardBody>
				<h2 className="text-6xl font-bold text-center">{visits}</h2>
			</CardBody>
		</Card>
	);
}
