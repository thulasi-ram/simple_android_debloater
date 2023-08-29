export type Package = {
	name: string;
	ptype: string;
	state: string;
	package_prefix: string;
};

export type DeviceUserPackages = {
	device_id: string;
	user_id: string;
	packages: Package[];
};

export type DeviceUserPackage = {
	device_id: string;
	user_id: string;
	package: Package;
};

export type Config = {
	prompt_disable_package: boolean;
};

export type Label = {
	name: string;
	description: string;
};

export type PackageDiscussion = {
	id: string;
	title: string;
	closed: boolean;
	body: string;
	bodyHTML: string;
	answer: any;
	labels: Label[];
};

export type PackageDiscussions = Record<string, PackageDiscussion>;
