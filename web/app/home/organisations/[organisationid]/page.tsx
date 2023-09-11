import React from "react";
import { data } from "@/lib/data";
import Icons from "@/components/icons";
import { Button } from "@/components/ui/Button";
import Projects from "@/components/ui/Projects";
import {
	Avatar,
	AvatarFallback,
	AvatarImage,
} from "@/components/ui/Avatar"
import Link from "next/link";

async function getData(id: string) {
	// get org data from api
	console.log(id);
	return data.find((org) => org.id === id);
}

export default async function Page({ params }: { params: { organisationid: string } }) {
	const org = await getData(params.organisationid);

	const handleFilterProject = (query: string) => {
		console.log(query);
	};

	const handleProjectClick = () => {
		console.log("project clicked");
	};
	// console.log(org);

	return (
		<main className="flex flex-col mt-10 bg-transparent">
			<div className="flex flex-row justify-around items-start">
				<div className="flex flex-row gap-3 items-center">
					<Avatar>
						<AvatarImage src={org?.logo} alt={`logo of organisation`} />
						<AvatarFallback>
							<Icons.DefaultOrganisation />
						</AvatarFallback>
					</Avatar>
					<Link href={`/home/organisations`}>
						<h1 className="text-xl font-semibold"> {org?.title} </h1>
					</Link>
				</div>
				<div className="flex flex-col w-2/5 justify-around gap-10">
					{
						org?.projects.length !== 0 && (
							<Projects org={org || null} />
						)
					}
				</div>
				<Button className="rounded-md bg-[#376789] text-white" asChild>
					<Link href={`/home/organisations/${org?.id}/projects/new`}>
						<Icons.PlusIcon /> Add Projects
					</Link>
				</Button>
			</div>
			{org?.projects.length === 0 && (
				<div className="flex flex-col items-center gap-4 my-auto w-full">
					<Icons.NotFound />
					<p className="text-xl w-fit font-medium">
						Oops! nothing found. Get started by creating new Project
					</p>
				</div>
			)}
		</main>
	);
}

export async function generateStaticParams() {
	// const posts = await fetch('https://.../posts').then((res) => res.json())
	return data.map((org) => ({
		params: {
			organisationid: org.id,
		},
	}));
}
