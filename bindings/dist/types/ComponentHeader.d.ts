import type { ComponentAccessRules } from "./ComponentAccessRules";
import type { ComponentBody } from "./ComponentBody";
import type { EntityId } from "./EntityId";
import type { OwnerRule } from "./OwnerRule";
export interface ComponentHeader {
    template_address: Uint8Array;
    module_name: string;
    owner_key: string | null;
    owner_rule: OwnerRule;
    access_rules: ComponentAccessRules;
    entity_id: EntityId;
    body: ComponentBody;
}
