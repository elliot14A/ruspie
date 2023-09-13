"use client";
import React, { FC } from "react";
import { Button } from "./Button";
import Icons from "../icons";
import { Organisation } from "@/types/organisation";
import { SearchBar } from "./searchBar";
import DeleteButttonWithConfirmModal from "./DeleteButttonWithConfimModal";
import Link from "next/link";

interface OrganisationProps {
	org: Organisation | null;
}

const Project: FC<OrganisationProps> = ({ org }) => {
	const handleFilterProject = (query: string) => {
		//
		console.log(query);
	};

	const handleEditClick = () => {
		console.log("edit clicked");
	};

	const handleDeleteClick = (event: React.MouseEvent<HTMLElement>) => {
		event.stopPropagation();
		event.nativeEvent.preventDefault();
	};

	return (
		<>
			<SearchBar
				placeholder="Search Organisation"
				callback={handleFilterProject}
			/>
			<div className="flex flex-col items-center">
				{org?.projects?.map((project, index) => (
					<Link
						href={`/home/organisations/${org?.id}/projects/${project.id}`}
						className={`flex flex-row justify-between items-center bg-white w-full p-4 border-b border-[#EAECF0]
								${index === 0 ? "rounded-t-md" : ""}
								${org !== null &&
								org.projects !== undefined &&
								index === org.projects.length - 1
								? "rounded-b-md"
								: ""
							}
							`}
						key={org.id + "_" + project.id + "_" + project.title}
					>
						<div className="flex flex-col gap-2">
							<h3 className="text-md">{project.title}</h3>
							<p className="text-sm text-[#6B7280]">
								Created at:{" "}
								<span className="text-[#6B7280] font-semibold">
									{project.createdAt}
								</span>
							</p>
						</div>
						<div className="flex gap-2">
							<Button
								variant="outline"
								size="icon"
								className="rounded border border-[#E6E6E6]"
								onClick={handleEditClick}
							>
								<Icons.EditIcon />
							</Button>
							<DeleteButttonWithConfirmModal
								onButtonClick={handleDeleteClick}
								onConfirm={async () => { }}
								onCancel={() => { }}
							/>
						</div>
					</Link>
				))}
			</div>
		</>
	);
};

export default Project;