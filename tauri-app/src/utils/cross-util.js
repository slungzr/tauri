export function getPhaseInfo(cross, stageNo) {
    const { stageParamList, signalGroupParamList, lampGroupList } = cross;
    // 创建快速查找的映射表
    const sgpMap = new Map();
    signalGroupParamList.forEach(sgp => {
        sgpMap.set(sgp.signalGroupNo, sgp);
    });

    const lampGroupMap = new Map();
    lampGroupList.forEach(lg => {
        lampGroupMap.set(lg.lampGroupNo, lg);
    });

    const result = {};

    if (stageNo) {
        // 处理指定阶段的情况
        const phaseList = [];
        for (const stageParam of stageParamList) {
            if (stageParam.stageNo.toString() == stageNo) {
                const sgsList = stageParam.signalGroupStatusList.signalGroupStatus;
                for (const sgs of sgsList) {
                    const sgp = sgpMap.get(sgs.signalGroupNo);
                    if (sgp) {
                        for (const grpNo of sgp.lampGroupNoList.lampGroupNo) {
                            const lampGroup = lampGroupMap.get(parseInt(grpNo));
                            if (lampGroup) {
                                const status = sgs.lampStatus.substring(2);
                                if (status === '2') {
                                    const mov = getMovementType(lampGroup.type.toString());
                                    if (mov) {
                                        phaseList.push({
                                            dir: lampGroup.direction.toString(),
                                            mov: mov
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
                break;
            }
        }
        result.phaseList = phaseList;
    } else {
        // 处理所有阶段的情况
        const allPhases = {};
        for (const stageParam of stageParamList) {
            const currentStageNo = stageParam.stageNo.toString();
            const phaseList = [];
            
            const sgsList = stageParam.signalGroupStatusList.signalGroupStatus;
            for (const sgs of sgsList) {
                const sgp = sgpMap.get(sgs.signalGroupNo);
                if (sgp) {
                    for (const grpNo of sgp.lampGroupNoList.lampGroupNo) {
                        const lampGroup = lampGroupMap.get(parseInt(grpNo));
                        if (lampGroup) {
                            const status = sgs.lampStatus.substring(2);
                            if (status === '2') {
                                const mov = getMovementType(lampGroup.type.toString());
                                if (mov) {
                                    phaseList.push({
                                        dir: lampGroup.direction.toString(),
                                        mov: mov
                                    });
                                }
                            }
                        }
                    }
                }
            }
            allPhases[currentStageNo] = phaseList;
        }
        result.allPhases = allPhases;
    }

    return result;
}

// 辅助函数（需要根据实际类型映射实现）
export function getMovementType(type) {
    switch (type) {
        case "10": return "直右";
        case "21": return "直";
        case "22": return "左"; 
        case "23": return "右";
        case "40": return "都";
        case "41": return "进";
        case "42": return "出";
        default: return "";
    }
}