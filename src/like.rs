// use crate::database::conn::DbConn;
// use diesel::prelude::*;
// use std::collections::HashMap;

// pub fn get_like(user_id: i32, conn: &DbConn) -> Vec<i32> {
//     let users = crate::admin::shop::logic::get_num_like(100, conn);
//     let goods = crate::admin::shop::logic::get_num_goods(100, conn);
//     let usmo_list = HashMap::new();


// }
// #（1）数据输入。
// users = ["贝贝", "晶晶", "欢欢", "迎迎", "妮妮"]  
// movies = ["战狼2", "哪吒之魔童转世", "流浪地球", "红海行动", "唐人街探案2", "美人鱼", "我和我的祖国"]  
// UsMoList=[  
//             [1, 1, 1, 0, 1, 0, 0],  
//             [0, 1, 1, 0, 0, 1, 0],  
//             [1, 0, 1, 1, 1, 1, 1],  
//             [1, 1, 1, 1, 1, 0, 0],  
//             [1, 1, 0, 1, 0, 1, 1]]  
   
// #（2）将行转为列
// def RowConverCol():  
//     return np.array(UsMoList).transpose().tolist()  
   
// #（3）使用欧氏距离计算两个电影之间的相似度。
// def euc_mv_sim(movieFirst: list, movieSecond:list):
//     return np.sqrt(((np.array(movieFirst) - np.array(movieSecond)) ** 2).sum())  
   
// #（4）计算所有电影之间的相似度。
// def allmv_sim():  
//     resDic = {}  
//     tempList = RowConverCol()  
//     for i in range(0, len(tempList)):  
//         for j in range(i+1, len(tempList)):  
//             resDic[str(i) + '-' + str(j)] = euc_mv_sim(tempList[i], tempList[j])  
//     return resDic  
   
//   #（5）计算要推荐哪些电影。 
// def comput_Rec_mo(username: str) -> list:  
//     temp = {}  
//     mo_sim_dic = allmv_sim()  
//     userindex = users.index(username)  
//     TargetUsermovieList = UsMoList[userindex]  
//     for i in range(0, len(TargetUsermovieList)):  
//         for j in range(i+1, len(TargetUsermovieList)):  
//             if TargetUsermovieList[i] == 1 and TargetUsermovieList[j] == 0 and (mo_sim_dic.get(str(i) + '-' + str(j)) != None or mo_sim_dic.get(str(j) + '-' + str(i)) != None):  
//                 sim = mo_sim_dic.get(str(i) + '-' + str(j)) if(mo_sim_dic.get(str(i) + '-' + str(j)) != None) else mo_sim_dic.get(str(j) + '-' + str(i))  
//                 temp[j] = sim  
//             elif TargetUsermovieList[i] == 0 and TargetUsermovieList[j] == 1 and (mo_sim_dic.get(str(i) + '-' + str(j)) != None or mo_sim_dic.get(str(j) + '-' + str(i)) != None):
//                 sim = mo_sim_dic.get(str(i) + '-' + str(j)) if (mo_sim_dic.get(str(i) + '-' + str(j)) != None) else mo_sim_dic.get(str(j) + '-' + str(i))  
//                 temp[i] = sim  
//     temp = sorted(temp.items(), key=lambda d:d[1])  
//     #temp = sorted(temp.items()) 
//     print("推荐列表：",temp)  
//     recommendlist = [movies[i] for i,v in temp]  
//     print("电影推荐：", recommendlist)  
//     return recommendlist  
// print(allmv_sim())  
